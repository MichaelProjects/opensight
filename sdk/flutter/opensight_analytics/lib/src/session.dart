import 'dart:isolate';

import 'persistence.dart';
import 'package:flutter_isolate/flutter_isolate.dart';

int trackIntervall = 5;

class Session {
  /// [Session] is used to count teh session length and later add more features to it.
  int length = 0;

  void increaseLength() {
    length += trackIntervall;
  }

  int store() {
    return length;
  }
}

/// the [trackIntervall] in which distance the data will be written to the disk or storage
startTracking(SendPort sendPort) async {
  var session = Session();
  while (true) {
    await Future.delayed(Duration(seconds: trackIntervall));
    session.increaseLength();
    await PresistencesLayer().storeSession(session);
  }
}

Future sendReceive(SendPort port, msg) {
  ReceivePort response = ReceivePort();
  port.send([msg, response.sendPort]);
  return response.first;
}

/// if [tracking] is called, the function will create an isolate and
/// start an endless loop to write down the past time
tracking() async {
  ReceivePort receivePort = ReceivePort();
  await FlutterIsolate.spawn(startTracking, receivePort.sendPort);
}
