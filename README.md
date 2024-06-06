# tms_keycmd

SSH AuthorizedKeysCommand for Trust Manager System (TMS)

## Program tms_keycmd

Command line program to support the SSH AuthorizedKeysCommand option for
retrieving authorized public keys for a user during ssh login.
 
This program accepts 4 arguments and calls the Trust Manager System (TMS)
server to fetch the associated public key. Public keys are looked up using
the login username and fingerprint of the public key.
If a public key is found it is written to stdout.
If no public key is found then nothing is written to stdout.
All other output is written to the log file.

The following 4 arguments must be passed in on the command line:

 - %u - login username (used in key lookup)
 - %U - numeric login user id (info only)
 - %f - fingerprint of the public key to be fetched (used in key lookup)
 - %t - ssh key type (info only)

Example:

```
   tms_keycmd.sh jdoe 1001 SHA256:I/YLbfco8m4WWZSDSNZ/OnV26tt+BgtFAcAb94Co974 ssh-rsa
```

The main program is written in rust. There is also a wrapper shell script that allows
the program to be invoked from an arbitrary location and still locate the tms_keycmd
configuration files.

## Building

Building the program uses the standard rust ecosystem. For a debug version run
``cargo build``. For the release version run ``cargo build --release``.

## Installation

Currently, installation is a manual process. Once the program binary is built, the
binary, shell script and two configuration files must be copied to the desired
location. For example, if all files are located in the current directory and the
desired installation directory is ``/etc/ssh/tms_keycmd``, then as root run commands
similar to the following:

```
mkdir -p /etc/ssh/tms_keycmd
cp tms_keycmd tms_keycmd.sh log4rs tms_keycmd.toml /etc/ssh/tms_keycmd/
cd /etc/ssh/tms_keycmd
chmod +x tms_keycmd tms_keycmd.sh
```

## Configuration of tms_keycmd

The program reads its settings from the configuration file ``tms_keycmd.toml``.
There are four entries in this file and they must all be modified. The entries
are:

- tms_url: The URL for the TMS server
- host_name: The host name to use when calling the TMS server to fetch a public key.
- client_id: Id of the client that is registered with the TMS server.
- client_secret: Credential for the client that is registered with the TMS server.

For example:

```
tms_url="https://tms-test-server1.tacc.utexas.edu:32201/v1/tms/creds/publickey"
host_name="stampede3"
client_id="tacc_tapis"
client_secret="*****************"
```

Note that the attribute ``host_name`` does not need to match the host name reported
by the operating system. It is up to the TMS administrator to set the host name
correctly. Having this attribute independent of the host name reported by the
operating system allows for supporting a machine that might be referenced using
many different host names, such as ``login1.stampede3.tacc.utexas.edu``,
``stampede3.tacc.utexas.edu``, ``stampede3``, ``129.114.63.133``, etc.
For example, for the Tapis v3 deployment at TACC, any Tapis system registered using
any of the aforementioned host names will route to the same machine.

## Configuration of SSHD

To configure a host to make use of tms_keycmd, the sshd configuration file on
the host must be modified. The configuration file may be found, for example,
at ``/etc/ssh/sshd_config``.

Two settings must be updated, *AuthorizedKeysCommand* and *AuthorizedKeysCommandUser*.
If no *AuthorizedKeysCommand* has been configured previously, the two entries in the
configuration file might look like this:

```
#AuthorizedKeysCommand none
#AuthorizedKeysCommandUser nobody
```

To configure sshd for tms_keycmd, uncomment both lines and replace ``none`` with the
command used to invoke the tms_keycmd program. For example, if tms_keycmd is installed
in directory ``/etc/ssh/tms_keycmd``, the updated lines would look like this:

```
AuthorizedKeysCommand /etc/ssh/tms_keycmd/tms_keycmd.sh %u %U %f %t
AuthorizedKeysCommandUser nobody
```
