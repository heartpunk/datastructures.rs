use Sys::Ptrace qw(ptrace PTRACE_ATTACH PT_STEP);
use File::Slurp;

my $pid = $ARGV[0];

print "ptrace return value = " . ptrace(PTRACE_ATTACH, $pid) . "\n";
waitpid($pid, WIFSTOPPED);

print "past wait\n";
# sleep 1000;
for (my $i = 0; $i <= 10; $i++) {
	ptrace(PT_STEP, $pid);
	print `python mem_dump.py $pid`;
}

print "past loop\n";
# ptrace(PTRACE_DETACH, $pid);
`kill -9 $pid`;