task :build do ||
	sh "cargo build"
end
	
task :test => [:build, :clean_test_junk] do ||
	sh "cargo test -- --nocapture"
end

task :clean_test_junk do ||
	sh "rm -rf test_db"
end