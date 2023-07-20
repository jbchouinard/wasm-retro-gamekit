dist: dist/bouncybox.tar.gz dist/gameoflife.tar.gz

dist/bouncybox.tar.gz:
	mkdir -p dist
	$(MAKE) -C demos/bouncybox dist.tar.gz
	cp demos/bouncybox/dist.tar.gz dist/bouncybox.tar.gz

dist/gameoflife.tar.gz:
	mkdir -p dist
	$(MAKE) -C demos/gameoflife dist.tar.gz
	cp demos/gameoflife/dist.tar.gz dist/gameoflife.tar.gz

dist/wrgedit.tar.gz:
	mkdir -p dist
	$(MAKE) -C tools/wrgedit dist.tar.gz
	cp tools/wrgedit/dist.tar.gz dist/wrgedit.tar.gz

serve: dist/bouncybox.tar.gz dist/gameoflife.tar.gz dist/wrgedit.tar.gz
	cd dist; \
		mkdir -p www/bouncybox; \
		tar -xf bouncybox.tar.gz -C www/bouncybox; \
		mkdir -p www/gameoflife; \
		tar -xf gameoflife.tar.gz -C www/gameoflife; \
		mkdir -p www/wrgedit; \
		tar -xf wrgedit.tar.gz -C www/wrgedit; \
		cd www; \
		python -m http.server 8080

clean:
	rm -rf ./dist
	$(MAKE) -C demos/bouncybox clean
	$(MAKE) -C demos/gameoflife clean
	cargo clean

.PHONY: dist clean serve
