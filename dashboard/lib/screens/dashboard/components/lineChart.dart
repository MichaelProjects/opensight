import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';

class LineChartCard extends StatefulWidget {
  String title;
  List<FlSpot> data;
  LineChartCard(this.title, this.data);

  @override
  State<LineChartCard> createState() => _LineChartCardState();
}

class _LineChartCardState extends State<LineChartCard> {
  @override
  Widget build(BuildContext context) {
    return Container(
        child: Padding(
            padding: const EdgeInsets.only(right: 18, top: 10, bottom: 18),
            child: LineChart(
              LineChartData(
                lineBarsData: [
                  LineChartBarData(
                    colors: const [Color(0xFFFF26B5), Color(0xFFFF5B5B)],
                    dotData: FlDotData(show: false),
                    belowBarData: BarAreaData(
                      show: true,
                      colors: const [Color(0x10FF26B5), Color(0x00FF26B5)],
                      gradientFrom: const Offset(0.5, 0),
                      gradientTo: const Offset(0.5, 1),
                    ),
                    spots: widget.data,
                  )
                ],
                maxY: 10,
                gridData: FlGridData(show: false),
                borderData: FlBorderData(show: false),
              ),
            )));
  }
}
