.PHONY: build rsync deploy

build:
	yarn build

rsync:
	rsync -az --force --delete --progress -e ssh ./build/ sigma:/var/www/solsensor.com/html

deploy: build rsync
