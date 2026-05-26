begin
  do_something
rescue => e
  handle_error(e)
ensure
  cleanup
end
