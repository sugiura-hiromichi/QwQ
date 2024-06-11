#!/bin/sh
date >> /var/log/foo3.log
df -HT >> /var/log/foo3.log
tail -f /dev/null
