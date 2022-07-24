#!/bin/sh

ln -sfr hooks/pre-commit.hook .git/hooks/pre-commit || exit
chmod +x .git/hooks/pre-commit
