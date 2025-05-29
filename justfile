# just command line runner commands for the astronomy-lover-net-v1 repo

# use Powershell instead of sh
set shell := ["powershell.exe", "-c"]

# start up a development server
dev:
  cd frontend; trunk serve

# build for production
build:
  cd frontend; trunk build --release

# deploy to shuttle
deploy:
  shuttle deploy
