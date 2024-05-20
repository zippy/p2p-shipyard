## NixOS

### Connect to devices

```bash
sudo adb devices
```

### Firewall

Disable the firewall to get the tauri frontend with:

1. Identify firewall rule number: 

```bash
sudo iptables -L INPUT --line-numbers
```

2. Remove firewall rule:

```bash
sudo iptables -D INPUT <RULE_NUM>
```
