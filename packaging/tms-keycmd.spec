Name:           tms-keycmd
Version:        0.1.1
Release:        1%{?dist}
Summary:        TMS KeyCmd utility program
BuildArch:      x86_64
#BuildArch:      aarch64

License:        BSD-3-Clause
URL:            https://tms-documentation.readthedocs.io/en/latest/index.html
Source0:        %{name}-%{version}.tgz

#BuildRequires:  
Requires:       bash

%description
Trust Manager System (TMS) program to support the SSH AuthorizedKeysCommand
option for retrieving authorized public keys.

%prep
%autosetup


%install
rm -rf $RPM_BUILD_ROOT
mkdir -p $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd/logs
cp -r * $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd
#cp tms_keycmd $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd
#cp tms_keycmd.sh $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd
#cp tms_keycmd.toml $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd
#cp log4rs.yml $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd
#cp README.md $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd
#cp LICENSE $RPM_BUILD_ROOT/%{_sysconfdir}/ssh/tms_keycmd

%clean
rm -fr $RPM_BUILD_ROOT

%files
%dir %attr(711,root,nobody) %{_sysconfdir}/ssh/tms_keycmd
%dir %attr(711,nobody,nobody) %{_sysconfdir}/ssh/tms_keycmd/logs
%attr(711,nobody,nobody) %{_sysconfdir}/ssh/tms_keycmd/logs/tms_keycmd.log
%attr(751,root,nobody) %{_sysconfdir}/ssh/tms_keycmd/tms_keycmd.sh
%attr(711,nobody,nobody) %{_sysconfdir}/ssh/tms_keycmd/tms_keycmd
%attr(600,nobody,nobody) %{_sysconfdir}/ssh/tms_keycmd/tms_keycmd.toml
%attr(600,nobody,nobody) %{_sysconfdir}/ssh/tms_keycmd/log4rs.yml
%attr(-,nobody,nobody) %license %{_sysconfdir}/ssh/tms_keycmd/LICENSE
%attr(-,nobody,nobody) %{_sysconfdir}/ssh/tms_keycmd/README.md

%changelog
* Thu Mar 20 2025 scblack
- Initial version.
