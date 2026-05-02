# nginx-infra-tutorial

Docker compose: tool to simplify the process of defining and running multiple containers 

docker-compose: create and start all containers in the background 


nginx: acts as a load balancer and secure the access and do all sorts of configurations before it reaches the backend severs. 

Paths to nginx-related files:

logs:
 C:\Users\User\AppData\Local\Microsoft\WinGet\Packages\nginxinc.nginx_Microsoft.Winget.Source_8wekyb3d8bbwe\nginx-1.29.8\logs\access.log


config:

C:\Users\User\AppData\Local\Microsoft\WinGet\Packages\nginxinc.nginx_Microsoft.Winget.Source_8wekyb3d8bbwe\nginx-1.29.8\conf\nginx.conf


502 Bad Gateway: backend is down -- nginx cannot serve requests 

ssl enables encryption by using public-key cryptography 
- when a user connects to a website via https, the web server provides its ssl certificate, which contains a public key 
- the client uses the public key to establish a secure, encrypted session with the server 



openssl req -x509 -nodes -days 365 -newkeys rsa:2048 -keyout nginx-selfsigned.key -out nginx-self-signed.crt


private key: kept secret and never shared publicly. used to decrypt data 

- public key is shared with clients to encrypt data 


443: the standard port for https traffic, enabling ssl for secure communication 

check whether any nginx processes are running: 

- tasklist | findstr nginx
- Get-Process nginx -ErrorAction SilentlyContinue
- netstat -ano | findstr :8080 (checks if port 8080 is listening)