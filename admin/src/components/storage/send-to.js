import React, { useState, useEffect } from "react";
import axios from "axios";
import { Col, Row, message, Modal } from "antd";
import myImage from "../../assets/images/Hard-Drive3.png";
import driver from "../../assets/images/Hard-Drive.png";
import { FiX } from "react-icons/fi";
import Cookies from "js-cookie";

const SendTO = ({ selected, uuid, fetchData, showModal, handleCancel }) => {
  // -------state management ---------------

  const [, setLoading] = useState(false);
  const [storages, setStorages] = useState([]);

  // -------token ----------
  const baseUrl = process.env.REACT_APP_API_URL;
  // const getToken = localStorage.getItem("token");
  const getToken = Cookies.get("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  useEffect(() => {
    setLoading(true);
    axios({
      method: "GET",
      url: `${baseUrl}/settings/storage/status`,
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setStorages(res.data);
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
      .post(`${baseUrl}/settings/storage/device/copy_or_move`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        if ((res.statusCode = 200)) {
          setLoading(true);
          message.success("Successful!");
          fetchData();
          handleCancel();
          setLoading(false);
        } else {
          message.error("Operation Failed! ");
        }
      })
      .catch((err) => {
        console.log(err);
      });
  };

  return (
    <React.Fragment>
      <Modal
        visible={showModal}
        footer={null}
        closeIcon={<FiX className="close-icon" onClick={handleCancel} />}
        width={350}
      >
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
                    <p className="send-to-sub-space">
                      {res.free_space} free of {res.total_space}
                    </p>
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
                    <p className="send-to-sub-space">
                      {res.free_space} free of {res.total_space}
                    </p>
                  </Col>
                </Row>
              </div>
            );
          }
        })}
      </Modal>
    </React.Fragment>
  );
};

export default SendTO;
