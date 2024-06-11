from centos:7
run yum install -y motif tigervnc-server xterm xorg-x11-fonts-Type1 vlgothic-fonts dejavu-sans-fonts dejavu-serif-fonts \
&& mkdir -p /root/.vnc && echo password | /usr/bin/vncpasswd -f > /root/.vnc/passwd && chmod 600 /root/.vnc/passwd
copy xstartup /root/.vnc
copy vncstart.sh /
run chmod +x /root/.vnc/xstartup && chmod +x /vncstart.sh
entrypoint ["/vncstart.sh"]
expose 5902 5903 5904
