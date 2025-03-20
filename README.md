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

The program may be run with the single command line argument ``--version``.
In this case it will print the version to the console and exit.

## Building

Building the program uses the standard rust ecosystem. For a debug version run
``cargo build``. For the release version run ``cargo build --release``.

## Packaging and Installation

To build and package the program simply run the script located at *deployment/build_package.sh*.
This will create the file ``tms-keycmd-<version>.tgz`` in the current working directory.

To install the program on a host, copy the tar file to the host and unpack it in the desired location.
For example, if the tar file is at ``/tmp/tms-keycmd-0.1.0.tgz`` and the desired installation directory is
``/etc/ssh/tms_keycmd``, then as root run commands similar to the following:

```
# Use nogroup for ubuntu and nobody for CentOS/Rocky
export NOGROUP=nogroup
cd /etc/ssh
tar -xvf /tmp/tms-keycmd-0.1.0.tgz
mv tms-keycmd-0.1.0 tms_keycmd
cd tms_keycmd
chown nobody:${NOGROUP} -R *
chown root:${NOGROUP} . tms_keycmd.sh
chmod go-r . *
chmod go-w .
chmod +x tms_keycmd tms_keycmd.sh
chmod g+r tms_keycmd.sh
```

Then be sure to configure *tms_keycmd* and *SSHD* using the instructions below.
In particular, remember to update the files *tms_keycmd.toml* and *sshd_config* before
the final step of re-starting the sshd service.

Note that ownership is changed to the user *nobody* because we will configure *SSHD* to run as this user.
The *tms_keycmd* program must have its configuration files owned by the user running the program.
If *SSHD* is configured to run as a user other than *nobody* then you must update ownership to that user.
Note also that *SSHD* requires that the *AuthorizedKeysCommand* be owned by ``root`` and not writable by
group or others.

### File permissions and ownership

Once the program is installed, ownership and permissions should look similar to the following:

```
# pwd
/etc/ssh/tms_keycmd
# ls -la
drwx--x--x  3 root   nogroup    4096 Mar 12 13:25 .
drwxr-xr-x 13 root   root       4096 Mar 12 13:30 ..
-rw-------  1 nobody nogroup    6342 Mar 12 13:25 LICENSE
-rw-------  1 nobody nogroup    6342 Mar 12 13:25 README.md
-rw-------  1 nobody nogroup     680 Mar 12 13:25 log4rs.yml
drwx--x--x  2 nobody nogroup    4096 Mar 12 13:27 logs
-rwx--x--x  1 nobody nogroup 3290712 Mar 12 13:25 tms_keycmd
-rwxr-x--x  1 root   nogroup    1282 Mar 12 13:25 tms_keycmd.sh
-rw-------  1 nobody nogroup     111 Mar 12 13:28 tms_keycmd.toml
```

### Note on shared library compatibility

Please note that it may be necessary to build the *tms_keycmd* binary on the target host.
Building on the target host should ensure that all required shared libraries are available
and compatible.

## Configuration of tms_keycmd

The program reads its settings from the configuration file ``tms_keycmd.toml``.
There are two entries in this file and they must both be modified. The entries
are:

- tms_url: The URL for the TMS server
- host_name: The host name to use when calling the TMS server to fetch a public key.

For example:

```
tms_url="https://tms-test-server1.tacc.utexas.edu:32201/v1/tms/pubkeys/creds/retrieve"
host_name="stampede3"
```

Note that the attribute ``host_name`` does not need to match the host name reported
by the operating system. It is up to the TMS administrator to set the host name
correctly. Having this attribute independent of the host name reported by the
operating system allows for supporting a machine that might be referenced using
many different host names, such as ``login1.stampede3.tacc.utexas.edu``,
``stampede3.tacc.utexas.edu``, ``stampede3``, ``129.114.63.133``, etc.
For example, for the Tapis v3 deployment at TACC, any Tapis system registered using
any of the aforementioned host names will route to the same cluster.

## Manual testing

It is advisable to manually test the *tms_keycmd* binary once it is installed and configured.
Often when the the program fails the *sshd* log will not be helpful. To test, use the TMS server
to create an ssh keypair. Using the fingerprint from the generated key, run a command similar
to the following:

```
/etc/ssh/tms_keycmd/tms_keycmd.sh testuser2 1003 <fingerprint> ssh-ed25519
```

The output should be a single line containing the public key from the generated keypair.


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

Note that *SSHD* requires that the *AuthorizedKeysCommand* be owned by ``root`` and not writable by
group or others.

Finally, as user root, re-start the sshd service:

```
systemctl restart sshd.service
```
