Name:           tms-keycmd
Version:        0.1.1
Release:        1%{?dist}
Summary:        TMS KeyCmd utility program
ExclusiveArch:  x86_64
#ExclusiveArch:  aarch64

License:        BSD-3-Clause
URL:            https://tms-documentation.readthedocs.io/en/latest/index.html
Source0:        %{name}-%{version}.tgz

#BuildRequires:
#Requires:       bash

%description
Trust Manager System (TMS) program to support the SSH AuthorizedKeysCommand
option for retrieving authorized public keys.

%prep
%autosetup


# NOTE: During cleanup only remove specific dir to avoid interfering with others.
%install
rm -fr $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd
mkdir -p $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd/logs
cp -r * $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd

%clean
rm -fr $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd

%files
%dir %attr(711,root,root) %{_sysconfdir}/ssh/tms_keycmd
%dir %attr(711,nobody,nobody) %{_sysconfdir}/ssh/tms_keycmd/logs
%attr(711,nobody,nobody) %{_sysconfdir}/ssh/tms_keycmd/logs/tms_keycmd.log
%attr(755,root,nobody) %{_sysconfdir}/ssh/tms_keycmd/tms_keycmd.sh
%attr(755,root,nobody) %{_sysconfdir}/ssh/tms_keycmd/tms_keycmd
%attr(600,nobody,nobody) %{_sysconfdir}/ssh/tms_keycmd/tms_keycmd.toml
%attr(600,nobody,nobody) %{_sysconfdir}/ssh/tms_keycmd/log4rs.yml
%attr(644,nobody,nobody) %license %{_sysconfdir}/ssh/tms_keycmd/LICENSE
%attr(644,nobody,nobody) %{_sysconfdir}/ssh/tms_keycmd/README.md

%changelog
* Thu Apr 17 2025 scblack
- Initial version.
