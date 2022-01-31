import 'package:coolicons/coolicons.dart';
import 'package:dashboard/controllers/app_controller.dart';
import 'package:dashboard/model/application.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

/// The [Appdetails] widget is a sidebar element that displays the application information
/// that where specified.
class Appdetails extends StatefulWidget {
  final Application? app;
  Appdetails({this.app});

  @override
  _AppdeatilsState createState() => _AppdeatilsState();
}

class _AppdeatilsState extends State<Appdetails> {
  @override
  Widget build(BuildContext context) {
    ApplicationProvider appController =
        Provider.of<ApplicationProvider>(context);
    return Container(
        margin: const EdgeInsets.all(10),
        padding: const EdgeInsets.only(left: 20, top: 10, bottom: 5),
        decoration: BoxDecoration(
          color: Theme.of(context).primaryColor,
          borderRadius: BorderRadius.circular(5),
        ),
        child: Builder(builder: (context) {
          /// Determines if it has an app as arugment or not

          if (widget.app != null) {
            return Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                visual(widget.app!),
                Text(
                  widget.app!.name,
                  overflow: TextOverflow.ellipsis,
                  style: Theme.of(context).textTheme.headline5,
                ),
                Text(widget.app!.packageId,
                    overflow: TextOverflow.ellipsis,
                    style: Theme.of(context).textTheme.subtitle1),
              ],
            );
          }
          // fetch data from backend if argument is null
          else {
            switch (appController.appStatus) {
              case AppStatus.loading:
                {
                  return const Center(child: CircularProgressIndicator());
                }
              case AppStatus.loaded:
                {
                  return Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      visual(appController.selectedApp),
                      Text(
                        appController.selectedApp.name,
                        overflow: TextOverflow.ellipsis,
                        style: Theme.of(context).textTheme.headline5,
                      ),
                      Text(appController.selectedApp.packageId,
                          overflow: TextOverflow.ellipsis,
                          style: Theme.of(context).textTheme.subtitle1),
                    ],
                  );
                }
              default:
                {
                  {
                    return const Center(
                      child: Text(
                        'Error',
                        style: TextStyle(color: Colors.red),
                      ),
                    );
                  }
                }
            }
          }
        }));
  }
}

Widget visual(Application app) {
  return Row(
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
      app.isAndroid == true
          ? Icon(Coolicons.google, color: Colors.grey[700])
          : Container(),
    ],
  );
}
