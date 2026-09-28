/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The root of the extraction library, so `lake build` builds all of it.

Lake's default target builds this module, and for a while it listed five of the
library's modules while more existed. The ones it left out were never elaborated:
`lake build` reported success without checking them, and so did the CI job that
runs it. Every generated and refinement module belongs here, because one that is
not imported below is checked by nothing.
-/

import NonosExtraction.Align
import NonosExtraction.AlignRefinement
import NonosExtraction.Caps
import NonosExtraction.CapsComplete
import NonosExtraction.CapsCoreRefinement
import NonosExtraction.Closure
import NonosExtraction.Ct
import NonosExtraction.CtPrimitivesRefinement
import NonosExtraction.CtRefinement
import NonosExtraction.CtEqRefinement
import NonosExtraction.EdField
import NonosExtraction.Elf
import NonosExtraction.ElfRefinement
import NonosExtraction.Iommu
import NonosExtraction.IommuRefinement
import NonosExtraction.Irq
import NonosExtraction.IrqRefinement
import NonosExtraction.Paging
import NonosExtraction.PagingRefinement
import NonosExtraction.PolicyRefinement
import NonosExtraction.Refinement
import NonosExtraction.RvFlags
import NonosExtraction.RvFlagsRefinement
import NonosExtraction.Shapes
import NonosExtraction.Signal
import NonosExtraction.SignalRefinement
import NonosExtraction.Uefi
import NonosExtraction.UefiRefinement
import NonosExtraction.Vectors
import NonosExtraction.VectorsRefinement
