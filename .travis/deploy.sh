#!/bin/bash

if [ $TRAVIS_BRANCH == "master" ] ; then

    eval "$(ssh-agent -s)"
    ssh-add ~/.ssh/travis_rsa
    git remote add deploy "travis@scarletbus.com:/ScarletBus/api/"
    git config user.name "Travis CI"
    git config user.email "travis@scarletbus.com"

    git add .
    git status # debug
    git commit -m "deploy"
    git push -f deploy HEAD:master
fi
