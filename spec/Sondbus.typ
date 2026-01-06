#set page(
  paper: "a4",
  header: align(left)[
    The Sondbus Protocol Suite
  ],
)

#show title: set text(size: 50pt)
#show heading: set text(size: 20pt)
#show heading.where(level: 1): set text(size: 35pt)
#show heading.where(level: 2): set text(size: 25pt)

#set text(
  size: 12pt,
)

#set heading(
  numbering: "1.1.1.",
  outlined: true,
)

#set par(
  justify: true,
)

#title[The Sondbus Protocol Suite]
#pagebreak()
#outline()
#pagebreak()

#include "About.typ"

#pagebreak()
#include "Architecture.typ"

#pagebreak()
#include "Transfer-Layer.typ"
