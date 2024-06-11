from centos:7
run yum install -y epel-release nginx iproute
run mkdir -p /etc/nginx/conf.d /var/www/{abcd,efgh}
run echo "ABCD" > /var/www/abcd/index.html
run echo "EFGH" > /var/www/efgh/index.html
copy vhost.conf /etc/nginx/conf.d/
cmd ["/usr/sbin/nginx","-g","daemon off;"]
