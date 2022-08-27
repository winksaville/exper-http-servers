# Experiment with http servers

[Searched](https://www.google.com/search?q=http+server+example+rust) and
found [this](https://gist.github.com/mjohnsullivan/e5182707caf0a9dbdf2d).

This is "working" it's able to receive metrics and not drop the connection.
The last and proble main issue was the 200 response was incorrect. I needed
to provide a header with "Content-Length: 0" and a blank line after the
header.

```
wink@3900x 22-08-27T05:29:30.602Z:~/prgs/rust/myrepos/exper-http-servers/httpsrvr1 (main)
$ cargo run 0.0.0.0:8099
   Compiling httpsrvr1 v0.1.0 (/home/wink/prgs/rust/myrepos/exper-http-servers/httpsrvr1)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `/home/wink/prgs/rust/myrepos/exper-http-servers/target/debug/httpsrvr1 '0.0.0.0:8099'`
Listening for connections on addr 0.0.0.0:8099
main: TOL for stream
in-comming: call spawning
in-comming: retf spawning
main: BOL for stream
main: spawn:+
looping_client:+
looping_client: TOL
read_header:+ ***
read_header: create locals
read_header: read cmd
read_header: cmd='POST / HTTP/1.1
'
BODY_LEN_RE match: Captures({0: Some("content-length: 1011"), 1: Some("1011")})
read_header: blank line, headers done
read_header:-
cmd: POST / HTTP/1.1 body_len: 1011
hdrs:
'[
    "content-type: application/json",
    "accept: */*",
    "host: 192.168.1.101:8099",
    "content-length: 1011",
]'
body: len=1011
'[{"version":1,"timestamp":1661579241208,"process":"beaconnode","cpu_process_seconds_total":28,"memory_process_bytes":981352448,"client_name":"lighthouse","client_version":"3.0.0","client_build":0,"disk_beaconchain_bytes_total":104054596491,"sync_eth1_connected":true,"sync_eth1_fallback_configured":false},{"version":1,"timestamp":1661579241228,"process":"system","cpu_cores":4,"cpu_threads":8,"cpu_node_system_seconds_total":1672831,"cpu_node_user_seconds_total":132806,"cpu_node_iowait_seconds_total":3295,"cpu_node_idle_seconds_total":1524595,"memory_node_bytes_total":33523871744,"memory_node_bytes_free":1312202752,"memory_node_bytes_cached":27771305984,"memory_node_bytes_buffers":576520192,"disk_node_bytes_total":982820896768,"disk_node_bytes_free":165996326912,"disk_node_io_seconds":0,"disk_node_reads_total":6441943,"disk_node_writes_total":6323176,"network_node_bytes_total_receive":41876115608,"network_node_bytes_total_transmit":67918640182,"misc_node_boot_ts_seconds":1661369409,"misc_os":"lin"}]'
handle_write:+
Response sent: 'HTTP/1.1 200 OK
Content-Length: 0

'
handle_write:-
looping_client: TOL
read_header:+ ***
read_header: create locals
read_header: read cmd
read_header: cmd='POST / HTTP/1.1
'
BODY_LEN_RE match: Captures({0: Some("content-length: 1188"), 1: Some("1188")})
read_header: blank line, headers done
read_header:-
cmd: POST / HTTP/1.1 body_len: 1188
hdrs:
'[
    "content-type: application/json",
    "accept: */*",
    "host: 192.168.1.101:8099",
    "content-length: 1188",
]'
body: len=1188
'[{"version":1,"timestamp":1661579301236,"process":"beaconnode","cpu_process_seconds_total":61,"memory_process_bytes":1118019584,"client_name":"lighthouse","client_version":"3.0.0","client_build":0,"disk_beaconchain_bytes_total":104044966563,"network_libp2p_bytes_total_receive":1627299,"network_libp2p_bytes_total_transmit":2400974,"network_peers_connected":20,"sync_beacon_head_slot":3755939,"sync_eth1_connected":true,"sync_eth1_fallback_configured":false,"sync_eth2_synced":true},{"version":1,"timestamp":1661579301256,"process":"system","cpu_cores":4,"cpu_threads":8,"cpu_node_system_seconds_total":1673310,"cpu_node_user_seconds_total":132837,"cpu_node_iowait_seconds_total":3296,"cpu_node_idle_seconds_total":1525039,"memory_node_bytes_total":33523871744,"memory_node_bytes_free":1074712576,"memory_node_bytes_cached":27773644800,"memory_node_bytes_buffers":577523712,"disk_node_bytes_total":982820896768,"disk_node_bytes_free":166004895744,"disk_node_io_seconds":0,"disk_node_reads_total":6442689,"disk_node_writes_total":6329054,"network_node_bytes_total_receive":41885105455,"network_node_bytes_total_transmit":67925131512,"misc_node_boot_ts_seconds":1661369409,"misc_os":"lin"}]'
handle_write:+
Response sent: 'HTTP/1.1 200 OK
Content-Length: 0

'
handle_write:-
looping_client: TOL
read_header:+ ***
read_header: create locals
read_header: read cmd
```

And the here is a body above pretty printed with jq:
```
wink@3900x 22-08-27T06:08:22.403Z:~/prgs/rust/myrepos/exper-http-servers (main)
$ cat data.txt | jq .
[
  {
    "version": 1,
    "timestamp": 1661579301236,
    "process": "beaconnode",
    "cpu_process_seconds_total": 61,
    "memory_process_bytes": 1118019584,
    "client_name": "lighthouse",
    "client_version": "3.0.0",
    "client_build": 0,
    "disk_beaconchain_bytes_total": 104044966563,
    "network_libp2p_bytes_total_receive": 1627299,
    "network_libp2p_bytes_total_transmit": 2400974,
    "network_peers_connected": 20,
    "sync_beacon_head_slot": 3755939,
    "sync_eth1_connected": true,
    "sync_eth1_fallback_configured": false,
    "sync_eth2_synced": true
  },
  {
    "version": 1,
    "timestamp": 1661579301256,
    "process": "system",
    "cpu_cores": 4,
    "cpu_threads": 8,
    "cpu_node_system_seconds_total": 1673310,
    "cpu_node_user_seconds_total": 132837,
    "cpu_node_iowait_seconds_total": 3296,
    "cpu_node_idle_seconds_total": 1525039,
    "memory_node_bytes_total": 33523871744,
    "memory_node_bytes_free": 1074712576,
    "memory_node_bytes_cached": 27773644800,
    "memory_node_bytes_buffers": 577523712,
    "disk_node_bytes_total": 982820896768,
    "disk_node_bytes_free": 166004895744,
    "disk_node_io_seconds": 0,
    "disk_node_reads_total": 6442689,
    "disk_node_writes_total": 6329054,
    "network_node_bytes_total_receive": 41885105455,
    "network_node_bytes_total_transmit": 67925131512,
    "misc_node_boot_ts_seconds": 1661369409,
    "misc_os": "lin"
  }
]
```

## xtask scripts

The following sections define tasks, "scripts" written in rust,
which maybe executed with either `cargo xtask xxx` or `cargo xt xxx`.
Where `xxx` is one of the `Tasks` below:

> See [cargo/config](.cargo/config) for the aliases

Tasks
 * pre-commit
   * Runs `cargo fmt`, `cargo clippy` and `cargo test` in \<proj-root\>

 * gen-phl
   * Removes <proj-root>/coverage/ then generates coverage data in <proj-root>/coverage/
   using gen-profraw, gen-html and gen-lcov.
   [Click to see coverage/html](https://htmlpreview.github.io/?https://github.com/winksaville/workspace-template-with-xtask/blob/main/coverage/html/index.html)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

