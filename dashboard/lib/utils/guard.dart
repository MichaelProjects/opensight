import 'package:flutter/material.dart';
import 'package:routemaster/routemaster.dart';

class AuthGuard {
  static bool isAppSelected = false;
  static bool canUserAccessPage() {
    return true;
  }

  static bool canUserAccessDashboard() {
    if (isAppSelected) {
      return true;
    }
    return false;
  }
}
