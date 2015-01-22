task :build do ||
	sh "cargo build"
end
	
task :test => [:build, :clean_test_junk] do ||
	sh "cargo test"
end

task :clean_test_junk do ||
	sh "rm -rf test_db"
end