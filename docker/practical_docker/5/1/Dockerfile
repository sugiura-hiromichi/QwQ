from centos:7
run yum install -y mariadb-server mariadb-client
copy server.cnf /etc/my.cnf.d/
copy mariadb.sh /mariadb.sh
run chmod +x /mariadb.sh
volume ["/var/lib/mysql"]
expose 3306
entrypoint ["/mariadb.sh"]
