from centos:7
run yum install -y httpd iproute
onbuild copy default1.html /var/www/html/
onbuild copy default2.html /var/www/html/
cmd ["usr/sbin/httpd","-DFOREGROUND"]
