build:
	wasm-pack build --release
	cd www; npm install; npm run build

dist.tar.gz: build
	tar cvzf dist.tar.gz -C www/dist .

start:	
	wasm-pack build --profiling
	cd www; npm install; npm run start

clean:
	rm -rf ./pkg ./www/dist ./www/node_modules ./www/package-lock.json

.PHONY: build start clean
