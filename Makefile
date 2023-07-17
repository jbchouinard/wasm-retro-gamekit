dist: dist/bouncybox.tar.gz dist/gameoflife.tar.gz

dist/bouncybox.tar.gz:
	mkdir -p dist
	$(MAKE) -C demos/bouncybox dist.tar.gz
	cp demos/bouncybox/dist.tar.gz dist/bouncybox.tar.gz

dist/gameoflife.tar.gz:
	mkdir -p dist
	$(MAKE) -C demos/gameoflife dist.tar.gz
	cp demos/gameoflife/dist.tar.gz dist/gameoflife.tar.gz

clean:
	rm -rf ./dist
	$(MAKE) -C demos/bouncybox clean
	$(MAKE) -C demos/gameoflife clean
	cargo clean

.PHONY: dist clean
