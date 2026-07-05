#!/bin/sh
cd ../data || exit
sea-orm-cli generate entity  -u "mysql://root:root123456@localhost:3306/scm" -o model/src/dao/
