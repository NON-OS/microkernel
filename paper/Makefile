# Build the 0.9.3 whitepaper.  From the repository root:  make -C paper
# Needs tectonic (or xelatex and bibtex) and python3. Fonts are in fonts/ (SIL OFL).
# The logo and diagram PDFs are converted from the SVGs with rsvg-convert.

all: tables main.pdf

tables:
	cd .. && python3 -I paper/tools/gen_tables.py

main.pdf: main.tex references.bib $(wildcard sections/*.tex) $(wildcard figures/*)
	tectonic -X compile main.tex

figures-pdf:
	for f in figures/diagram-*.svg logo/nonos-logo-cyan.svg; do rsvg-convert -f pdf -o $${f%.svg}.pdf $$f; done

clean:
	rm -f main.pdf main.aux main.bbl main.blg main.log main.out

.PHONY: all tables figures-pdf clean
