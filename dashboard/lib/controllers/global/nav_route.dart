import 'package:dashboard/screens/app_select/app_select.dart';
import 'package:dashboard/screens/dashboard/dashboard.dart';
import 'package:dashboard/utils/guard.dart';
import 'package:flutter/material.dart';
import 'package:routemaster/routemaster.dart';

import '../../screens/explore/explore.dart';

final routes = RouteMap(routes: {
  '/': (routeData) => AuthGuard.canUserAccessPage()
      ? MaterialPage(child: AppSelect())
      : const MaterialPage(
          child: Scaffold(
              body: Center(
                  child: Text('You are not authorized to access this page')))),
  '/dashboard/:id': (routeData) => AuthGuard.canUserAccessDashboard()
      ? const MaterialPage(child: const Dashboard())
      : const Redirect(
          "/",
        ),
  '/explore/:id': (routeData) => AuthGuard.canUserAccessDashboard()
      ? MaterialPage(child: Explore())
      : const Redirect(
          "/",
        )
});
