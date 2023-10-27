import sklearn as sk
import numpy   as np 
import pandas  as pd
import pyarrow as pa 




from sklearn.model_selection  import train_test_split, cross_val_score
from sklearn.metrics          import mean_squared_error

import sys
import json 

print(" Scikit-Learn Version: ", sk.__version__)
print(" Numpy Version: "       , np.__version__)
print(" PyArrow Version: "     , pa.__version__)
print(" Pandas Version: "      , pd.__version__)

interface_path = sys.argv[1]  # interface_file
model = sys.argv[2]           # model
parameters = sys.argv[3]      # params

print("Model {}, params {}".format(model,parameters))

print("Hello from Python >>>>>>\nOpening the following file in Python:", interface_path)

with pa.ipc.open_file(interface_path) as reader: #reading the arrow (.ipc file) generated in rust
   df = reader.read_pandas()

parameters = {
   "test_size": 0.2,
   "target": "medv"
}

X = df.drop(parameters["target"], axis = 1)
y = df[parameters["target"]]

X_train, X_test, y_train, y_test = train_test_split(X,y,test_size=parameters["test_size"])


algorithm = None
if model == "LinearRegression":
   from sklearn.linear_model     import LinearRegression
   algorithm = LinearRegression()

if model == "SupportVectorMachine":
   from sklearn import svm
   algorithm = svm.SVR()
  

algorithm.fit(X_train, y_train)

# verify
y_test_predict = algorithm.predict(X_test)
rmse = (np.sqrt(mean_squared_error(y_test, y_test_predict)))
r2 = round(algorithm.score(X_test, y_test),2)

ret = {
   "RMSE": rmse,
   "R2": r2
}
print("Writing back information about the evaluation metrics")
print("--------------------------------------")
print('{}'.format(ret))

print("\n")



# writing back the results into the interface file to be viewed in Rust 
with open(interface_path, "w") as interface_file: 
   json.dump(ret, interface_file)
 