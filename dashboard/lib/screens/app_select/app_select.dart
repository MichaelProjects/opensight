// ignore_for_file: prefer_const_constructors

import 'package:dashboard/controllers/app_controller.dart';
import 'package:dashboard/model/application.dart';
import 'package:dashboard/screens/app_select/components/appselector.dart';
import 'package:dashboard/screens/app_select/components/decoration_spacer.dart';
import 'package:dashboard/screens/overlay/topbar.dart';
import 'package:dashboard/utils/sizes.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class AppSelect extends StatefulWidget {
  AppSelect({Key? key}) : super(key: key);

  @override
  _AppSelectState createState() => _AppSelectState();
}

class _AppSelectState extends State<AppSelect> {
  List<Application> apps = [];
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance?.addPostFrameCallback((_) {
      var appProvider = Provider.of<ApplicationModel>(context, listen: false);
      appProvider.fetchApplications();
    });
  }

  @override
  Widget build(BuildContext context) {
    var data = MediaQuery.of(context).size;
    return Scaffold(
        body: Column(
      children: [
        const TopBar(),
        Stack(
          clipBehavior: Clip.none,
          children: [
            SizedBox(height: data.height - topbarHeight, width: data.width),
            DecorationSpacer(),
            Positioned(
                top: 80, left: (data.width / 100) * 43, child: AppSelector())
          ],
        )
      ],
    ));
  }
}
