cc = xelatex
dev:
	latexrun --latex-cmd=$(cc) main.tex
clean:
	latexrun --clean-all
watch:
	entr -s "latexrun --latex-cmd=$(cc) main.tex" main.tex
