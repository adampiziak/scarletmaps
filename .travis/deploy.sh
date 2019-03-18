#!/bin/bash

if [ $TRAVIS_BRANCH == "master" ] ; then
    eval "$(ssh-agent -s)"
    ssh-add ~/.ssh/travis_rsa

    git init
    git remote add deploy "travis@scarletbus.com:/ScarletBus/api/"
    git config user.name "Travis CI"
    git config user.email "travis@scarletbus.com"

    git add .
    git commit -m "Deploy"
    git push -f deploy master
fi
