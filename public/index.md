<!-- Creator     : groff version 1.23.0 -->
<!-- CreationDate: Sun Oct 29 11:02:41 2023 -->
<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN"
"http://www.w3.org/TR/html4/loose.dtd">
<html>
<head>
<meta name="generator" content="groff -Thtml, see www.gnu.org">
<meta http-equiv="Content-Type" content="text/html; charset=US-ASCII">
<meta name="Content-Style" content="text/css">
<style type="text/css">
       p       { margin-top: 0; margin-bottom: 0; vertical-align: top }
       pre     { margin-top: 0; margin-bottom: 0; vertical-align: top }
       table   { margin-top: 0; margin-bottom: 0; vertical-align: top }
       h1      { text-align: center }
</style>
<title></title>
</head>
<body>

<hr>


<p><i>CMD_CACHIER</i>(1) General Commands Manual
<i>CMD_CACHIER</i>(1)</p>

<p style="margin-top: 1em"><b>NAME</b></p>

<p style="margin-left:9%;">cmd_cachier &mdash; A caching
service for your cli commands using redis.</p>

<p style="margin-top: 1em"><b>SYNOPSIS</b></p>

<p style="margin-left:24%;"><b>cmd_cachier</b>
<i>SUBCOMMAND</i> [YOUR_COMMAND] <b><br>
cmd_cachier</b> <i>YOUR_COMMAND...</i></p>

<p style="margin-top: 1em"><b>PREREQUESITES</b></p>

<p style="margin-left:9%;">A redis server must be runnigng
on the local machine. On MacOS with homebrew.</p>

<p style="margin-left:9%; margin-top: 1em">Start the daemon
and enable on boot on ...</p>


<p style="margin-left:9%; margin-top: 1em"><b>MacOS(homebrew)</b></p>

<p style="margin-left:9%; margin-top: 1em">When redis is
installed (via homebrew) run:</p>

<p style="margin-left:17%;"><b>$ brew service start
redis</b></p>


<p style="margin-left:9%; margin-top: 1em"><b>Linux(systemd)</b></p>

<p style="margin-left:9%; margin-top: 1em">When redis is
installed run:</p>

<p style="margin-left:17%;"><b>$ systemctl enable redis
<br>
$ systemctl start redis</b></p>

<p style="margin-left:9%; margin-top: 1em">For security
reasons if you are only going to use redis for local
purposes. Like: Development, cmd_cachier etc.. Then you
should edit your <b>redis.conf</b> And uncomment the
following line (MacOS: /opt/homebrew/etc/redis.conf, Linux:
/etc/redis/redis.conf) :</p>

<p style="margin-left:9%; margin-top: 1em">From:</p>

<p style="margin-left:17%;"><b># bind 127.0.0.1 ::1 #
listens on loopback IPv4 and IPv6</b></p>

<p style="margin-left:9%;">To:</p>

<p style="margin-left:17%;"><b>bind 127.0.0.1 ::1 # listens
on loopback IPv4 and IPv6</b></p>

<p style="margin-top: 1em"><b>DESCRIPTION</b></p>

<p style="margin-left:9%;"><i>TODO:</i> Add description</p>

<p style="margin-left:9%; margin-top: 1em">The db state is
preserved throughout a desktop session as redis is a in
memory database. To keep commands refreshed you can manually
execute your command with the <b>save</b> subcommand. Or you
could refresh it programatically via systemd timers or
crontab (man crontab). <b>crontab</b></p>

<p style="margin-left:9%; margin-top: 1em">Run:</p>

<p style="margin-left:17%;"><b>$ crontab -e</b></p>

<p style="margin-left:9%; margin-top: 1em">Add the line for
an update every minute:</p>

<p style="margin-left:17%; margin-top: 1em"><b>* * * * *
cmd_cache save YOUR_COMMAND</b></p>

<p style="margin-left:9%; margin-top: 1em">Or every 5
minutes:</p>

<p style="margin-left:17%; margin-top: 1em"><b>*/5 * * * *
cmd_cache save YOUR_COMMAND</b></p>

<p style="margin-left:9%; margin-top: 1em">TODO: systemd
timer documentation</p>

<p style="margin-left:9%; margin-top: 1em">If no operands
are given, ... directory are displayed. If a subcommand is
givenn, ... If only your command is given, ...</p>

<p style="margin-top: 1em"><b>SUBCOMMANDS</b></p>

<p style="margin-left:9%;">The following subcommands are
available:</p>

<p style="margin-top: 1em"><b>save</b>
<i>YOUR_COMMAND...</i></p>

<p style="margin-left:19%;">Save command to database in any
case. Will overwrite if already in db.</p>

<p style="margin-top: 1em"><b>query</b>
<i>YOUR_COMMAND...</i></p>

<p style="margin-left:19%;">Try to query database. Returns
empty string if not in db.</p>

<p style="margin-top: 1em"><b>memory</b></p>

<p style="margin-left:19%; margin-top: 1em">Display memory
usage of saved commands.</p>

<p style="margin-top: 1em"><b>info</b></p>

<p style="margin-left:19%; margin-top: 1em">Info about
redis database.</p>

<p style="margin-top: 1em"><b>EXAMPLES</b></p>

<p style="margin-left:9%;">Cache list of files in $HOME</p>

<p style="margin-left:17%; margin-top: 1em"><b>$
cmd_cachier ls -1 $HOME</b></p>

<p style="margin-left:9%; margin-top: 1em">Cache list of
git projects</p>

<p style="margin-left:17%; margin-top: 1em"><b>$
cmd_cachier fd --glob .git --hidden --type d --max-depth 6
--prune</b></p>

<p style="margin-left:9%; margin-top: 1em">Additionally
remove the .git folders from the path:</p>

<p style="margin-left:17%; margin-top: 1em"><b>$
cmd_cachier fd --glob .git --hidden --type d --max-depth 6
--prune</b> | <b>while read -r line;do
tmp=&quot;${line%/}&quot;;echo ${tmp%/*} ;done</b></p>

<p style="margin-left:9%; margin-top: 1em">With fzf:</p>

<p style="margin-left:17%; margin-top: 1em"><b>$
cmd_cachier fd</b>. <b>--type d --max-depth 5 --hidden
$HOME</b> | <b>fzf</b></p>

<p style="margin-top: 1em"><b>TIPS AND TRICKS</b></p>

<p style="margin-left:9%;">Alias gp to cd into a selected
git project in the example above.</p>

<p style="margin-left:9%; margin-top: 1em">In your
.bashrc:</p>

<p style="margin-left:17%; margin-top: 1em"><b>alias
gp=&rsquo;cd $(cmd_cachier YOUR_SEARCH_COMMAND | fzf)
&rsquo;</b></p>

<p style="margin-left:9%; margin-top: 1em">Enforce refresh
of the search on demand (alias with gpup):</p>

<p style="margin-left:17%; margin-top: 1em"><b>alias
gpup=&rsquo;cd $(cmd_cachier save YOUR_SEARCH_COMMAND | fzf)
&rsquo;</b> GNU October 28, 2023 <i>CMD_CACHIER</i>(1)</p>
<hr>
</body>
</html>
