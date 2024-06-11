	N=10000000
	m=0
	call srand(time())
	do i=1,N
	x=rand()
	y=rand()
	if(x*x+y*y<1.0)then
		m=m+1
		end if
	end do
	write(6,*) (float(m)/float(N))*4.0
	end
