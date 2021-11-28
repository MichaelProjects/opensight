import 'package:coolicons/coolicons.dart';
import 'package:dashboard/model/application.dart';
import 'package:flutter/material.dart';

/// The [Appdetails] widget is a sidebar element that displays the application information
/// that where specified.
class Appdetails extends StatelessWidget {
  final Application app;
  const Appdetails(this.app, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    /// this widget contains the app icon an the plattform information
    /// like Apple(Ios) or Goole(Android)
    Widget visuals = Row(
      children: [
        Container(
          width: 35,
          height: 35,
          decoration: BoxDecoration(
              color: Colors.grey[300],
              borderRadius: const BorderRadius.all(Radius.circular(60))),
        ),
        const SizedBox(width: 10),
        app.isIos == true
            ? Icon(
                Coolicons.apple,
                color: Colors.grey[700],
              )
            : Container(),
        app.isIos == true
            ? Icon(Coolicons.google, color: Colors.grey[700])
            : Container(),
      ],
    );

    return Container(
        margin: const EdgeInsets.all(10),
        padding: const EdgeInsets.only(left: 20, top: 10, bottom: 10),
        height: 100,
        width: 210,
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(10),
          color: Theme.of(context).primaryColor,
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            visuals,
            Text(
              app.name,
              style: Theme.of(context).textTheme.headline6,
            ),
            Text(app.packageId, style: Theme.of(context).textTheme.subtitle1),
          ],
        ));
  }
}
