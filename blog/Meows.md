## nmap result
```bash
└─# nmap -A -T4 --top-ports 1000 10.129.76.144
Starting Nmap 7.94SVN ( https://nmap.org ) at 2024-09-28 13:26 UTC
Nmap scan report for 10.129.76.144
Host is up (0.24s latency).
Not shown: 999 closed tcp ports (reset)
PORT   STATE SERVICE VERSION
23/tcp open  telnet  Linux telnetd
Aggressive OS guesses: HP P2000 G3 NAS device (97%), Linux 5.0 (95%), Linux 5.0 - 5.4 (95%), OpenWrt Kamikaze 7.09 (Linux 2.6.22) (94%), OpenWrt 0.9 - 7.09 (Linux 2.4.30 - 2.4.34) (93%), OpenWrt White Russian 0.9 (Linux 2.4.30) (93%), Linux 4.15 - 5.8 (93%), Linux 5.4 (93%), Linux 5.3 - 5.4 (92%), Linux 2.6.32 (92%)
No exact OS matches for host (test conditions non-ideal).
Network Distance: 3 hops
Service Info: OS: Linux; CPE: cpe:/o:linux:linux_kernel

TRACEROUTE (using port 8080/tcp)
HOP RTT       ADDRESS
1   0.03 ms   172.17.0.1
2   238.66 ms 10.10.14.1
3   238.75 ms 10.129.76.144

OS and Service detection performed. Please report any incorrect results at https://nmap.org/submit/ .
Nmap done: 1 IP address (1 host up) scanned in 42.36 seconds
```

## telnet
```bash
└─# telnet 10.129.76.144
Trying 10.129.76.144...
Connected to 10.129.76.144.
Escape character is '^]'.

  █  █         ▐▌     ▄█▄ █          ▄▄▄▄
  █▄▄█ ▀▀█ █▀▀ ▐▌▄▀    █  █▀█ █▀█    █▌▄█ ▄▀▀▄ ▀▄▀
  █  █ █▄█ █▄▄ ▐█▀▄    █  █ █ █▄▄    █▌▄█ ▀▄▄▀ █▀█


Meow login: root
```

---

pwned

---

