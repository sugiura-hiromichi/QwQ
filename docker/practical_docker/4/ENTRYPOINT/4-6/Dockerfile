from centos:7
copy httpd_vsftpd.sh /usr/local/bin
run yum install -y httpd vsftpd \
&& chmod 755 /usr/local/bin/httpd_vsftpd.sh \
&& useradd -m ftpuser01 \
&& echo "password1234" | passwd --stdin ftpuser01 \
&& ln -s /home/ftpuser01 /var/www/html/ftpuser01
entrypoint ["/usr/local/bin/httpd_vsftpd.sh"]
