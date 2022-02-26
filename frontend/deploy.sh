#!/usr/bin/env sh

set -e

npm run build
cd dist

# if you are deploying to a custom domain
# echo 'www.example.com' > CNAME
git init
git add -A
git commit -m 'deploy'
git remote add origin https://github.com/z5208980/Artpay-Playground.git
git push -u origin master
