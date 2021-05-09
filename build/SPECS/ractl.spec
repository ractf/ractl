Name:		    ractl
Version:        0.0.1
Release:        1
Summary:        Install and manage RACTF

License:        AGPLv3
URL:            https://ractf.co.uk
Source0:	      ractl-%{version}.tar.gz
ExclusiveArch:	x86_64
BuildRoot:	    %{_tmppath}/%{name}-buildroot

%define _build_id_links none

%description
Install and manage RACTF

%prep
%setup -q

%install
mkdir -p "$RPM_BUILD_ROOT"
cp -R * "$RPM_BUILD_ROOT"

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
/usr/bin/ractl

