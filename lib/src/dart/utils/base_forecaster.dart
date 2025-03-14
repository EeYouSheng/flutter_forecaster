import 'package:flutter/services.dart';
import 'package:flutter_forecaster/src/dart/model/prediction_source.dart';
import 'package:flutter_forecaster/src/dart/utils/csv_converter.dart';

abstract class BaseForecaster {
  Future<List<PredictionSource>> initializePrediction({required List<PredictionSource> sourceData});

  Uint8List convertToCsv(List<PredictionSource> dataList) =>
      DataConverter(content: dataList.map((e) => e.csvData).toList()).convert;
}
