import 'package:dashboard/screens/app_select/app_select.dart';
import 'package:dashboard/screens/dashboard/dashboard.dart';
import 'package:dashboard/utils/guard.dart';
import 'package:flutter/material.dart';
import 'package:routemaster/routemaster.dart';

final routes = RouteMap(routes: {
  '/': (routeData) => AuthGuard.canUserAccessPage()
      ? MaterialPage(child: AppSelect())
      : MaterialPage(
          child: Scaffold(
              body: Center(
                  child: Text('You are not authorized to access this page')))),
  '/dashboard/:id': (routeData) => AuthGuard.canUserAccessDashboard()
      ? MaterialPage(child: Dashboard())
      : Redirect(
          "/",
        )
});
