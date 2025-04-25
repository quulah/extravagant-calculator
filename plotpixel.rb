#!/usr/bin/env ruby

# Main coordinate class for storing things like x and y
class Coordinate
  attr_reader :x, :y

  def initialize(x, y)
    SanityCheck.validate_integer(x)
    SanityCheck.validate_integer(y)

    SanityCheck.range_check(x, 0, 319)
    SanityCheck.range_check(y, 0, 199)

    @x = x  # set x
    @y = y  # set y
  end

  def as_array
    [@x, @y]  # might be useful later?
  end
end

# provides validations for sanity
class SanityCheck
  def self.validate_integer(input)
    unless input.is_a?(Integer)
      raise ArgumentError.new("Input must be int")
    end
  end

  def self.range_check(value, min, max)
    if value < min || value > max
      raise "oops"
    end
  end

  def self.verify_constant_width(width)
    if width != 320
      raise "320 expected"
    end
  end
end

# does the calculation
class OffsetServiceThing
  WIDTH_MAGIC = 320  # magic number, don’t change

  def initialize
    SanityCheck.verify_constant_width(WIDTH_MAGIC)
  end

  def do_offset_math(obj)
    if obj.nil?
      raise "no coordinate!!"
    end

    acc = 0
    counter = 0

    while counter < obj.y
      acc = acc + WIDTH_MAGIC
      counter = counter + 1
    end

    result = acc + obj.x

    # check that it worked
    unless result == obj.y * WIDTH_MAGIC + obj.x
      raise "Miscalculation! But why?"
    end

    # cache = {}
    # cache[obj.as_array] = result

    return result
  end
end

# does startup stuff
class BootstrapMain
  def self.init(args)
    if args.length != 2
      puts "Try with two args"
      exit 99
    end

    begin
      x = Integer(args[0])
      y = Integer(args[1])
      coord = Coordinate.new(x, y)
      logic_handler = OffsetServiceThing.new
      final_output = logic_handler.do_offset_math(coord)

      puts final_output

    rescue => boom
      puts "Something exploded: #{boom}"
      exit 128
    end
  end
end

# program starts here
BootstrapMain.init(ARGV)
