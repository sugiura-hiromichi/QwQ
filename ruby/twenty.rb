# frozen_string_literal: true

def asrt_eq(left, right)
	return if left == right

	puts "ERROR: asserting #{left} & #{right} has failed"
	exit
end
asrt_eq(Math.sqrt(4), 2)

# doc
class Greet
	attr_accessor :names

	def initialize(names = 'me')
		@names = names
	end

	def say_hi
		if @names.nil?
			puts '😵💫'
		elsif @names.respond_to? 'each'
			@names.each do |name|
				puts "Hi #{name}"
			end
		else
			puts "hel #{@names}"
		end
	end

	def say_bye
		if @names.nil?
			puts '😱'
		elsif @names.respond_to? 'each'
			puts "gb #{@names.join ','}"
		else
			puts "Bye #{@names}"
		end
	end
end

if __FILE__ == $PROGRAM_NAME
	greet = Greet.new

	greet.say_hi
	greet.say_bye
	asrt_eq(greet.instance_of?(Greet), true)

	greet.names = %w[you me rb py rs]
	greet.say_bye
	greet.say_hi

	greet.names = nil
	greet.say_bye
	greet.say_hi
end

puts '🫠'
