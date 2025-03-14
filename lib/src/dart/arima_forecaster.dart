import 'dart:convert';

import 'package:flutter_forecaster/src/dart/model/prediction_source.dart';
import 'package:flutter_forecaster/src/dart/utils/base_forecaster.dart';
import 'package:flutter_forecaster/src/rust/api/arima.dart';

class ArimaForecaster extends BaseForecaster {
  @override
  Future<List<PredictionSource>> initializePrediction({
    required List<PredictionSource> sourceData,
  }) async {
    final List<PredictionSource> predictedList = [];

    final convertedData = convertToCsv(sourceData);
    final salesPredictionJson = await predictSales(csvData: convertedData);

    final salesPredictionMap =
        jsonDecode(salesPredictionJson) as Map<String, dynamic>;

    try {
      final saleJsonList = salesPredictionMap['predictions'] as List<dynamic>;
      for (int i = 0; i < saleJsonList.length; i++) {
        final sale = saleJsonList[i];

        predictedList.add(PredictionSource(
          date: sourceData.last.date
              .copyWith(month: sourceData.last.date.month + i + 1),
          value: sale as double,
        ));
      }
    } catch (e) {
      rethrow;
    }

    return predictedList;
  }
}
