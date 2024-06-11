from centos:7
run yum install -y httpd iproute
run echo "Hello foreground Apache httpd" > /var/www/html/test.html
entrypoint ["/usr/sbin/httpd","-DFOREGROUND"]
