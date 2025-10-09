cc = xelatex
dev:
	./latexrun --latex-cmd=$(cc) --latex-args=--shell-escape main.tex
clean:
	./latexrun --clean-all
prod:
	mv ./set.tex ../ && ./latexrun --latex-cmd=$(cc) --latex-args=-halt-on-error main.tex && ./latexrun --latex-cmd=$(cc) --latex-args=-halt-on-error main.tex && mv ../set.tex ./set.tex
