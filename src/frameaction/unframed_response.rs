use super::FrameAction;

/// An unframed response, interleaving response data
/// in an unframed manner
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnframedResponse {
    /// Wait for the leading bytes to pass by until it is this slave's turn
    LeadingBytes {
        /// The remaining bytes
        remaining: u16,
        /// The budget of bytes for the next frame
        budget: u8,
        /// The amount of bytes that will follow after this slave's response
        following_bytes: u16,
    },
    /// The slave in its response state
    Response {
        /// The bytes remaining in the response, excluding the CRC
        remaining: u8,
        /// The bytes that will follow in after this slave's response
        following_bytes: u16,
    },
    /// Wait for the following bytes from the other slaves to pass by
    FollowingBytes {
        /// The bytes that are missing before finishing this unframed response
        remaining: u16,
    },
}

impl UnframedResponse {
    /// Create a new unframed response
    /// # Arguments
    /// * `leading` - The amount of leading bytes before this slave's response
    /// * `budget` - The budget this slave has to response excluding the CRC
    /// * `following` - The amount of following bytes after this slave's turn
    pub fn new(leading: u16, budget: u8, following: u16) -> FrameAction {
        if leading != 0 {
            FrameAction::UnframedResponse(Self::LeadingBytes {
                remaining: leading,
                budget,
                following_bytes: following,
            })
        } else if budget != 0 {
            FrameAction::UnframedResponse(Self::Response {
                remaining: budget,
                following_bytes: following,
            })
        } else if following != 0 {
            FrameAction::UnframedResponse(Self::FollowingBytes {
                remaining: following,
            })
        } else {
            FrameAction::None
        }
    }

    /// Poll this response, extracting another byte
    pub fn next(self) -> (FrameAction, Option<u8>) {
        match self {
            Self::LeadingBytes {
                remaining,
                budget,
                following_bytes,
            } => (
                FrameAction::UnframedResponse(match remaining - 1 {
                    0 => Self::Response {
                        remaining: budget,
                        following_bytes,
                    },
                    _ => Self::LeadingBytes {
                        remaining: remaining - 1,
                        budget,
                        following_bytes,
                    },
                }),
                None,
            ),
            Self::Response {
                remaining,
                following_bytes,
            } => (
                FrameAction::UnframedResponse(match remaining - 1 {
                    0 => Self::FollowingBytes {
                        remaining: following_bytes,
                    },
                    _ => Self::Response {
                        remaining: remaining - 1,
                        following_bytes,
                    },
                }),
                Some(0xFF),
            ),
            Self::FollowingBytes { remaining } => (
                match remaining - 1 {
                    0 => FrameAction::None,
                    _ => FrameAction::UnframedResponse(Self::FollowingBytes {
                        remaining: remaining - 1,
                    }),
                },
                None,
            ),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::frameaction::FrameAction;

    use super::UnframedResponse;

    #[test]
    fn handle() {
        let action = UnframedResponse::new(1, 1, 1);
        let action = match action {
            FrameAction::UnframedResponse(action) => action,
            _ => panic!("Action is not unframed response"),
        };

        //
        // Handle the first byte, going to the response state
        //
        let (action, response) = action.next();
        assert_eq!(response, None, "Responds when it shouldn't");
        let action = match action {
            FrameAction::UnframedResponse(action) => action,
            _ => panic!("Action is not unframed response"),
        };
        assert_eq!(
            action,
            UnframedResponse::Response {
                remaining: 1,
                following_bytes: 1
            },
            "Unframed response is not in response state"
        );

        //
        // Handle the next byte, should return one and go to following bytes state
        //
        let (action, response) = action.next();
        assert!(response.is_some(), "Does not respond when it should");
        let action = match action {
            FrameAction::UnframedResponse(action) => action,
            _ => panic!("Action is not unframed response"),
        };
        assert_eq!(action, UnframedResponse::FollowingBytes { remaining: 1 });

        //
        // Handle the one following byte, should return no frame action
        //
        let (action, response) = action.next();
        assert_eq!(response, None, "Responds when it shouldn't");
        assert_eq!(action, FrameAction::None);
    }

    #[test]
    fn no_leading() {
        let response = UnframedResponse::new(0, 1, 1);

        assert_eq!(
            response,
            FrameAction::UnframedResponse(UnframedResponse::Response {
                remaining: 1,
                following_bytes: 1
            })
        )
    }

    #[test]
    fn no_leading_and_payload() {
        let response = UnframedResponse::new(0, 0, 1);

        assert_eq!(
            response,
            FrameAction::UnframedResponse(UnframedResponse::FollowingBytes { remaining: 1 })
        )
    }

    #[test]
    fn no_leading_and_payload_and_following() {
        let response = UnframedResponse::new(0, 0, 0);

        assert_eq!(response, FrameAction::None)
    }
}
