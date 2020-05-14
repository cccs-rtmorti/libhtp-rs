%define name libhtp

Name: libhtp
Summary: Parses HTTP traffic.
Version: %{version}
Release: 1
License: BSD
Source0: %{name}-%{version}.tar.gz

%description
Rust implementation of the libhtp library.

%prep
%setup

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
/usr/local/
