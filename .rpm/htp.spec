%define name libhtp

Name: libhtp
Summary: Security-aware HTTP protocol parsing library FFI package
Version: %{version}
Release: 1
License: BSD
Source0: %{name}-%{version}.tar.gz

%description
%{summary}

%package devel
Summary: Security-aware HTTP protocol parsing library FFI package headers and libraries
Requires: %{name} = %{version}-%{release}
Requires: pkgconfig

%description devel
%{summary}

%prep
%setup -q

%build
make %{?_smp_mflags}

%install
%make_install

%post -p /sbin/ldconfig
%postun -p /sbin/ldconfig

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_libdir}/%{name}.so.*

%files devel
%{_libdir}/libhtp.so
%{_includedir}/htp/*.h
