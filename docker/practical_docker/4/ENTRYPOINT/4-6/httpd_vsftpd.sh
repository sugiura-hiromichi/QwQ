#!/bin/bash
/usr/bin/pkill vsftpd
/usr/sbin/httpd -k restart # start httpd daemon
/usr/sbin/vsftpd /etc/vsftpd/vsftpd.conf # start vsftpd daemon
tail -f /dev/null
