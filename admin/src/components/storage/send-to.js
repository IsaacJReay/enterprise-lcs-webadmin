import React, { useState, useEffect } from "react";
import axios from "axios";
import { Col, Row, message } from "antd";
import myImage from "../../assets/images/Hard-Drive3.png";
import driver from "../../assets/images/Hard-Drive.png";

const SendTO = ({ selected, uuid }) => {
  // -------state management ---------------

  const [, setLoading] = useState(false);
  const [storages, setStorages] = useState([]);

  // -------token ----------

  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  useEffect(() => {
    setLoading(true);
    axios({
      method: "GET",
      url: "http://10.42.0.188:8080/private/api/settings/storage/status",
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setStorages(res.data);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

  // ---------handle send to  ---------

  const handleSend = (values) => {
    const inputData = {
      operation: "move",
      source_uuid: uuid,
      source_items: [`${selected}`],
      destination_uuid: values,
      items_destination: "",
    };
    axios
      .post(
        "http://10.42.0.188:8080/private/api/settings/storage/device/copy_or_move",
        inputData,
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
        }
      )
      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
          window.location.reload();
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        console.log(err);
        message.error("Operation Failed! ");
      });
  };

  return (
    <React.Fragment>
      <div>
        {storages.map((res, index) => {
          if (index === 0) {
            return (
              <div
                className="item-storages"
                onClick={() => handleSend(res.drive_partuuid.drive_partuuid)}
              >
                <Row gutter={[24, 0]}>
                  <Col span={6}>
                    <img
                      src={myImage}
                      className="send-storages-icon"
                      alt="storages"
                    />
                  </Col>
                  <Col span={18}>
                    <p>{res.drive_label}</p>
                  </Col>
                </Row>
              </div>
            );
          } else {
            return (
              <div
                className="item-storages"
                onClick={() => handleSend(res.drive_partuuid.drive_partuuid)}
              >
                <Row gutter={[24, 0]}>
                  <Col span={6}>
                    <img
                      src={driver}
                      className="send-storages-icon"
                      alt="storages "
                    />
                  </Col>
                  <Col span={18}>
                    <p>{res.drive_label}</p>
                  </Col>
                </Row>
              </div>
            );
          }
        })}
      </div>
    </React.Fragment>
  );
};

export default SendTO;
