import 'package:coolicons/coolicons.dart';
import 'package:dashboard/controllers/dashboard/sidebar/app_controller.dart';
import 'package:dashboard/model/application.dart';
import 'package:dashboard/utils/api/client.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

/// The [Appdetails] widget is a sidebar element that displays the application information
/// that where specified.
class Appdetails extends StatefulWidget {
  Appdetails({Key? key}) : super(key: key);

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
        child: FutureBuilder(
            future: appController.fetchApplications(),
            builder: (context, AsyncSnapshot<Application> snap) {
              switch (appController.appStatus) {
                case AppStatus.loading:
                  {
                    return Center(child: CircularProgressIndicator());
                  }
                case AppStatus.loaded:
                  {
                    return Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        visual(snap.data!),
                        Text(
                          snap.data!.name,
                          style: Theme.of(context).textTheme.headline5,
                        ),
                        Text(snap.data!.packageId,
                            style: Theme.of(context).textTheme.subtitle1),
                      ],
                    );
                  }
                default:
                  {
                    {
                      return Center(
                        child: Text(
                          'Error',
                          style: TextStyle(color: Colors.red),
                        ),
                      );
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
